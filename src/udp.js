import {invoke} from "@tauri-apps/api/core";

const init_button = document.getElementById("init");

init_button.addEventListener("click", (event) => {
    invoke("initialize_udp", {
        localAddr: "0.0.0.0:0",
        remoteAddr: document.getElementById("ipaddr").value,
    }).then((result) => {
        window.location.href="capture.html";
    });
});
