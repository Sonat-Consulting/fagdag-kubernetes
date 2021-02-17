const express = require("express");
const bodyParser = require('body-parser');
const moment = require('moment');

const app = express();
app.use(bodyParser.urlencoded({ extended: true }));
app.listen(3001, () => {
    console.log("Server running on port 3001");
});

app.use(function(req, res, next) {
    res.header("Access-Control-Allow-Origin", "*"); // update to match the domain you will make the request from
    next();
});


function randomColorSingle() {
    const r = Math.trunc(Math.random()*100);
    const n = 255 - r;
    return n.toString(16);
}

function randomColor() {
    const r = randomColorSingle();
    const g = randomColorSingle();
    const b = randomColorSingle();
    return `#${r}${g}${b}`;
}

const color = randomColor();

const randomId = Math.trunc(Math.random()*1000);


app.get("/", (req, res, next) => {
    
    let html = "<html>" +
        "<body>" +
        `<h1 style='background-color: ${color}'>I am an App running in Kubernetes</h1>` +
        "<p>My unique ID is <b>" + randomId + "</b></p>" +
        "</body>" +
        "</html>";

    res.send(html);
});


//app.use('/sumo-api-notification-simple/admin', express.static('public'))