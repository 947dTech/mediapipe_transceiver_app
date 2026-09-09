import {invoke} from "@tauri-apps/api/core";

const init_button = document.getElementById("init");
const send_button = document.getElementById("send");

init_button.addEventListener("click", (event) => {
    invoke("initialize_udp", {
        localAddr: "0.0.0.0:0",
        remoteAddr: document.getElementById("ipaddr").value,
    }).then();
});

send_button.addEventListener("click", (event) => {
    invoke("send_udp", {
        message: document.getElementById("message").value,
    }).then();
});
