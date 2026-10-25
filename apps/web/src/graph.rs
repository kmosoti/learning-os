//! Deterministic renderer experiment, separate from the lesson's small graph.
use leptos::prelude::*;
#[island]
pub fn GraphBenchmark() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    leptos::task::spawn_local(async {
        if let Err(e) = initialize() {
            web_sys::console::error_1(&e);
        }
    });
    view! { <main class="graph-benchmark"><h1>"LearningOS graph renderer experiment"</h1><p>"Seed 42 · the same nodes, joint-premise arguments, labels and camera in both renderers."</p><div id="benchmark-stage" data-ready="false"><canvas id="benchmark-canvas" width="1000" height="600" aria-label="Graph visualization; equivalent structured list below"></canvas><svg id="benchmark-svg" viewBox="0 0 1000 600" role="img" aria-label="Graph visualization; equivalent structured list below"></svg></div><details><summary>"Structured nodes and joint arguments"</summary><ol id="benchmark-list"></ol></details></main> }
}
#[cfg(feature = "hydrate")]
mod rendering {
    use std::{cell::RefCell, rc::Rc};
    use wasm_bindgen::{JsCast, prelude::*};
    #[derive(serde::Serialize)]
    struct Node {
        id: usize,
        x: f64,
        y: f64,
    }
    thread_local! {
    static NODES:RefCell<(usize,Rc<Vec<Node>>)>=RefCell::new((0,Rc::new(vec![])));
    static SVG:RefCell<Option<SvgState>>=const{RefCell::new(None)};
    }
    fn fixture(count: usize) -> Rc<Vec<Node>> {
        NODES.with(|cache| {
            if cache.borrow().0 == count {
                return cache.borrow().1.clone();
            }
            let mut seed = 42_u64;
            let nodes: Vec<Node> = (0..count)
                .map(|id| {
                    seed = seed
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    let x = ((seed >> 32) % 1920) as f64;
                    seed = seed
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    Node {
                        id,
                        x,
                        y: ((seed >> 32) % 1080) as f64,
                    }
                })
                .collect();
            let nodes = Rc::new(nodes);
            *cache.borrow_mut() = (count, nodes.clone());
            nodes
        })
    }
    fn document() -> web_sys::Document {
        web_sys::window()
            .expect("window")
            .document()
            .expect("document")
    }
    pub(super) fn initialize() -> Result<(), JsValue> {
        let doc = document();
        let params = web_sys::UrlSearchParams::new_with_str(
            &web_sys::window().expect("window").location().search()?,
        )?;
        let count = params
            .get("nodes")
            .and_then(|n| n.parse().ok())
            .filter(|n| [1000, 10000].contains(n))
            .unwrap_or(1000);
        let renderer = params.get("renderer").unwrap_or_else(|| "svg".into());
        let list = doc
            .get_element_by_id("benchmark-list")
            .expect("semantic list");
        for node in fixture(count).iter() {
            let li = doc.create_element("li")?;
            li.set_text_content(Some(&format!(
                "Claim {} at ({}, {})",
                node.id, node.x, node.y
            )));
            list.append_child(&li)?;
        }
        for i in (2..count).step_by(3) {
            let li = doc.create_element("li")?;
            li.set_text_content(Some(&format!(
                "Argument {}: claim {} AND claim {} jointly support claim {}",
                i / 3,
                i - 2,
                i - 1,
                i
            )));
            list.append_child(&li)?;
        }
        graph_frame(&renderer, count, 0.0, 0.0, 0.5)?;
        let stage = doc.get_element_by_id("benchmark-stage").expect("stage");
        stage.set_attribute("data-ready", "true")?;
        stage.set_attribute("data-nodes", &count.to_string())?;
        stage.set_attribute("data-renderer", &renderer)?;
        stage.set_attribute(
            "data-dpr",
            &web_sys::window()
                .expect("window")
                .device_pixel_ratio()
                .to_string(),
        )?;
        Ok(())
    }
    #[wasm_bindgen]
    pub fn graph_fixture_json(count: usize) -> Result<String, JsValue> {
        if ![1000, 10000].contains(&count) {
            return Err(JsValue::from_str("supported fixtures: 1000 or 10000"));
        }
        serde_json::to_string(fixture(count).as_ref())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    struct SvgState {
        count: usize,
        root: web_sys::Element,
        nodes: Vec<web_sys::Element>,
        texts: Vec<web_sys::Element>,
        arguments: Vec<web_sys::Element>,
        node_visible: Vec<bool>,
        argument_visible: Vec<bool>,
        zoom: f64,
        labels: bool,
    }
    impl SvgState {
        fn create(svg: &web_sys::Element, nodes: &[Node]) -> Result<Self, JsValue> {
            let mut html = String::with_capacity(nodes.len() * 300);
            html.push_str(
                "<g id='graph-camera'><g stroke='#434343' stroke-width='.6' fill='none'>",
            );
            for i in (2..nodes.len()).step_by(3) {
                let a = &nodes[i - 2];
                let b = &nodes[i - 1];
                let c = &nodes[i];
                let (jx, jy) = ((a.x + b.x + c.x) / 3.0, (a.y + b.y + c.y) / 3.0);
                html.push_str(&format!("<g id='argument-{i}'><path vector-effect='non-scaling-stroke' d='M{} {}L{jx} {jy}L{} {}M{} {}L{jx} {jy}'/><path vector-effect='non-scaling-stroke' stroke='#b5a98a' stroke-width='3' stroke-linecap='square' d='M{jx} {jy}h.001'/></g>",a.x,a.y,c.x,c.y,b.x,b.y));
            }
            html.push_str(
                "</g><g fill='#b5a98a' font-family='Source Sans 3,sans-serif' font-size='10'>",
            );
            for n in nodes {
                html.push_str(&format!("<g id='node-{}' transform='translate({} {})'><circle r='.001' fill='none' stroke='#b5a98a' stroke-width='4' vector-effect='non-scaling-stroke'/><text id='label-{}' x='5' y='3'>Claim {}</text></g>",n.id,n.x,n.y,n.id,n.id));
            }
            html.push_str("</g></g>");
            // All labels/IDs are deterministic integers; source/model text never enters this SVG builder.
            svg.set_inner_html(&html);
            let doc = document();
            let node_elements = (0..nodes.len())
                .map(|i| doc.get_element_by_id(&format!("node-{i}")).expect("node"))
                .collect();
            let texts = (0..nodes.len())
                .map(|i| doc.get_element_by_id(&format!("label-{i}")).expect("label"))
                .collect();
            let arguments = (2..nodes.len())
                .step_by(3)
                .map(|i| {
                    doc.get_element_by_id(&format!("argument-{i}"))
                        .expect("argument")
                })
                .collect::<Vec<_>>();
            let count = nodes.len();
            Ok(Self {
                count,
                root: doc.get_element_by_id("graph-camera").expect("camera"),
                nodes: node_elements,
                texts,
                node_visible: vec![true; count],
                argument_visible: vec![true; arguments.len()],
                arguments,
                zoom: 0.0,
                labels: true,
            })
        }
    }
    fn visibility(
        element: &web_sys::Element,
        previous: &mut bool,
        next: bool,
    ) -> Result<(), JsValue> {
        if *previous != next {
            if next {
                element.remove_attribute("display")?;
            } else {
                element.set_attribute("display", "none")?;
            }
            *previous = next;
        }
        Ok(())
    }
    #[wasm_bindgen]
    pub fn graph_frame(
        renderer: &str,
        count: usize,
        camera_x: f64,
        camera_y: f64,
        zoom: f64,
    ) -> Result<f64, JsValue> {
        if ![1000, 10000].contains(&count)
            || !zoom.is_finite()
            || !(0.1..=4.0).contains(&zoom)
            || !camera_x.is_finite()
            || !camera_y.is_finite()
        {
            return Err(JsValue::from_str("invalid graph fixture/camera"));
        }
        let window = web_sys::window().expect("window");
        let performance = window.performance().expect("performance");
        let start = performance.now();
        let nodes = fixture(count);
        let doc = document();
        let svg = doc.get_element_by_id("benchmark-svg").expect("svg");
        let canvas = doc
            .get_element_by_id("benchmark-canvas")
            .expect("canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        let project = |n: &Node| ((n.x - camera_x) * zoom, (n.y - camera_y) * zoom);
        let visible =
            |x: f64, y: f64| (-80.0..=1080.0).contains(&x) && (-30.0..=630.0).contains(&y);
        match renderer {
            "canvas" => {
                svg.set_attribute("hidden", "")?;
                canvas.remove_attribute("hidden")?;
                let dpr = window.device_pixel_ratio();
                let width = (1000.0 * dpr) as u32;
                let height = (600.0 * dpr) as u32;
                if canvas.width() != width || canvas.height() != height {
                    canvas.set_width(width);
                    canvas.set_height(height);
                }
                let context = canvas
                    .get_context("2d")?
                    .expect("2d")
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
                context.set_transform(dpr, 0.0, 0.0, dpr, 0.0, 0.0)?;
                context.clear_rect(0.0, 0.0, 1000.0, 600.0);
                context.set_stroke_style_str("#434343");
                context.set_line_width(0.6);
                context.begin_path();
                let mut junctions = vec![];
                for i in (2..count).step_by(3) {
                    let (ax, ay) = project(&nodes[i - 2]);
                    let (bx, by) = project(&nodes[i - 1]);
                    let (cx, cy) = project(&nodes[i]);
                    let (jx, jy) = ((ax + bx + cx) / 3.0, (ay + by + cy) / 3.0);
                    if visible(jx, jy) || visible(cx, cy) || visible(ax, ay) || visible(bx, by) {
                        context.move_to(ax, ay);
                        context.line_to(jx, jy);
                        context.line_to(cx, cy);
                        context.move_to(bx, by);
                        context.line_to(jx, jy);
                        junctions.push((jx, jy));
                    }
                }
                context.stroke();
                context.set_fill_style_str("#b5a98a");
                for (x, y) in junctions {
                    context.fill_rect(x - 1.5, y - 1.5, 3.0, 3.0);
                }
                context.set_font("10px 'Source Sans 3',sans-serif");
                for n in nodes.iter() {
                    let (x, y) = project(n);
                    if visible(x, y) {
                        context.begin_path();
                        context.arc(x, y, 2.0, 0.0, std::f64::consts::TAU)?;
                        context.fill();
                        if zoom >= 0.45 {
                            context.fill_text(&format!("Claim {}", n.id), x + 5.0, y + 3.0)?;
                        }
                    }
                }
            }
            "svg" => {
                canvas.set_attribute("hidden", "")?;
                svg.remove_attribute("hidden")?;
                SVG.with(|cache| {
                    let mut cache = cache.borrow_mut();
                    if cache.as_ref().is_none_or(|s| s.count != count) {
                        *cache = Some(SvgState::create(&svg, &nodes)?);
                    }
                    let state = cache.as_mut().expect("retained SVG");
                    state.root.set_attribute(
                        "transform",
                        &format!(
                            "translate({} {}) scale({zoom})",
                            -camera_x * zoom,
                            -camera_y * zoom
                        ),
                    )?;
                    for (index, n) in nodes.iter().enumerate() {
                        let (x, y) = project(n);
                        visibility(
                            &state.nodes[index],
                            &mut state.node_visible[index],
                            visible(x, y),
                        )?;
                        if state.zoom != zoom {
                            state.texts[index]
                                .set_attribute("transform", &format!("scale({})", 1.0 / zoom))?;
                        }
                        if state.labels != (zoom >= 0.45) {
                            if zoom >= 0.45 {
                                state.texts[index].remove_attribute("display")?;
                            } else {
                                state.texts[index].set_attribute("display", "none")?;
                            }
                        }
                    }
                    for (index, i) in (2..count).step_by(3).enumerate() {
                        let (ax, ay) = project(&nodes[i - 2]);
                        let (bx, by) = project(&nodes[i - 1]);
                        let (cx, cy) = project(&nodes[i]);
                        let (jx, jy) = ((ax + bx + cx) / 3.0, (ay + by + cy) / 3.0);
                        visibility(
                            &state.arguments[index],
                            &mut state.argument_visible[index],
                            visible(jx, jy)
                                || visible(cx, cy)
                                || visible(ax, ay)
                                || visible(bx, by),
                        )?;
                    }
                    state.zoom = zoom;
                    state.labels = zoom >= 0.45;
                    Ok::<_, JsValue>(())
                })?;
            }
            _ => return Err(JsValue::from_str("renderer must be svg or canvas")),
        };
        Ok(performance.now() - start)
    }
}
#[cfg(feature = "hydrate")]
use rendering::initialize;
