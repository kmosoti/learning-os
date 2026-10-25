// Local test instrumentation for Composite UI Toolkit's documented text-spacing style.
// Production CSP stays unchanged. Only this exact style hash is allowed in the preview.
import http from 'node:http';
import {createHash} from 'node:crypto';
const target=new URL(process.env.LEARNINGOS_UI_TARGET||'http://127.0.0.1:3000');
if(target.protocol!=='http:'||!['127.0.0.1','localhost','[::1]'].includes(target.hostname))throw Error('Loopback HTTP target required');
const css='* {line-height:1.5!important;letter-spacing:.12em!important;word-spacing:.16em!important} p {margin-bottom:2em!important}';
const hash=createHash('sha256').update(css).digest('base64');
const server=http.createServer((req,res)=>{
 if(!['GET','HEAD'].includes(req.method)){res.writeHead(405);res.end();return;}
 const upstream=http.request({hostname:target.hostname,port:target.port,path:req.url,method:req.method,headers:{accept:req.headers.accept||'*/*','accept-encoding':req.headers['accept-encoding']||'identity'}},response=>{
  const headers={...response.headers};const csp=headers['content-security-policy'];
  if(csp)headers['content-security-policy']=csp.replace(/style-src 'self'(?=;|$)/,`style-src 'self' 'sha256-${hash}'`);
  res.writeHead(response.statusCode||502,headers);response.pipe(res);
 });upstream.on('error',e=>{res.writeHead(502);res.end(e.message)});upstream.end();
});
server.listen(Number(process.env.LEARNINGOS_SPACING_PORT||4301),'127.0.0.1',()=>console.log(`Spacing preview ${server.address().port}; exact style sha256-${hash}; target ${target.origin}`));
