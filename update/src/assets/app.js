window.addEventListener('load', function() {
    appendMsg('load');
});
window.addEventListener('error', function(e) { appendMsg('error: ' + JSON.stringify(e)) });
window.addEventListener('state-change', function(e) { appendMsg('state-change: ' + JSON.stringify(e)) });

function appendMsg(msg) {
    let msgDiv = document.createElement('div');
    let msgSpan = document.createElement('span');
    msgSpan.appendChild(document.createTextNode(msg));
    msgDiv.appendChild(msgSpan);
    document.body.appendChild(msgDiv);
}

function loop() {
    window.external.invoke('loop');
    setTimeout(loop, 2000);
}

loop();