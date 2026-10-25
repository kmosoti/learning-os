// Run under an isolated dbus-run-session + Xvfb with a real Orca process.
import {createRequire} from 'node:module';
import {writeFile,mkdir} from 'node:fs/promises';
const require=createRequire(process.env.LEARNINGOS_BROWSER_PACKAGE||import.meta.url);
const {firefox}=require('playwright');
const out=process.env.OUT||'artifacts/at';await mkdir(out,{recursive:true});
const browser=await firefox.launch({headless:false,firefoxUserPrefs:{'accessibility.force_disabled':0}});
const page=await browser.newPage({viewport:{width:1440,height:900}});const steps=[];
const mark=async name=>{steps.push({name,at:new Date().toISOString(),snapshot:await page.locator('body').ariaSnapshot()});await page.waitForTimeout(350);};
try{
 await page.goto((process.env.URL||'http://127.0.0.1:3000')+'/session/ownership');await page.locator('#main[data-ready=true]').waitFor();await page.bringToFront();if(process.env.ORCA_WAIT_MS)await page.waitForTimeout(Number(process.env.ORCA_WAIT_MS));
 await page.keyboard.press('Tab');await mark('skip-link');
 await page.getByRole('button',{name:'Start learning →',exact:true}).focus();await page.keyboard.press('Enter');await page.locator('#main[data-phase=Plan]').waitFor();await mark('plan');
 await page.getByRole('button',{name:'Make a prediction →'}).focus();await page.keyboard.press('Enter');await page.locator('#main[data-phase=Predict]').waitFor();await mark('prediction');
 await page.getByLabel("I don't know yet",{exact:true}).focus();await page.keyboard.press('Space');await mark('unknown-choice');
 await page.locator('#reasoning').focus();await page.keyboard.type('My prediction stays separate.');await mark('reasoning-field');
 await page.getByRole('button',{name:'Sources',exact:true}).focus();await page.keyboard.press('Enter');await page.getByRole('dialog').waitFor();await mark('source-dialog');await page.keyboard.press('Escape');await mark('dialog-dismissed');
 await page.getByRole('button',{name:'Save prediction →'}).focus();await page.keyboard.press('Enter');await page.locator('#main[data-phase=Teach]').waitFor();await mark('teaching');
 await page.getByRole('button',{name:'I’m ready to check →'}).focus();await page.keyboard.press('Enter');await page.locator('#main[data-phase=CheckClosed]').waitFor();await mark('closed-check');
 await page.getByRole('button',{name:'Open check →'}).focus();await page.keyboard.press('Enter');await page.locator('#main[data-phase=Check]').waitFor();await mark('open-check');
 await page.getByLabel('It does not compile',{exact:true}).focus();await page.keyboard.press('Space');await page.getByRole('button',{name:'Commit answer →'}).focus();await page.keyboard.press('Enter');await page.locator('#main[data-phase=Evidence]').waitFor();await mark('evidence');
 await writeFile(`${out}/steps.json`,JSON.stringify({browser:browser.version(),steps},null,2));
}finally{await browser.close();}
