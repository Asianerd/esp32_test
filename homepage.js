// const ESP_BACKEND_ADDRESS = 'http://127.0.0.1:8000';
const ESP_BACKEND_ADDRESS = 'https://www.ozmium.xyz/backend/esp';

function submitRequest() {
    sendGetRequest(`${ESP_BACKEND_ADDRESS}/set_current/${encodeURIComponent(document.querySelector("#submission").value)}`, (r) => {
        console.log(r);
        updateCurrentWord();
    })
}

function updateCurrentWord() {
    sendGetRequest(`${ESP_BACKEND_ADDRESS}/get_current`, (r) => {
        console.log(r);
        document.querySelector("#current-word").innerHTML = r;
    });
}

updateCurrentWord();
