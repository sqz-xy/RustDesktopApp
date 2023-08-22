 // access the pre-bundled global API functions
 const { invoke } = window.__TAURI__.tauri

 // now we can call our Command!
 // You will see "Welcome from Tauri" replaced
 // by "Hello, World!"!

 const input = document.getElementById("Input");
 input.addEventListener('keypress', function (e) {
    if (e.key === 'Enter') {
        invoke('save_command', { command: input.value });
        input.value = "";
    }
});

window.addEventListener("load", (event) => {
    invoke('load_database');    
});

 