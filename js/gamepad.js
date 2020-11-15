window.addEventListener("gamepadconnected", function(e) {
    console.log("Gamepad connected at index %d: %s",
      e.gamepad.index, e.gamepad.id);

    setInterval(pollGamepads, 500);
});

window.addEventListener("gamepaddisconnected", function(e) {
    console.log("Gamepad disconnected from index %d: %s",
      e.gamepad.index, e.gamepad.id);
});


// On frame update, poll the gamepads current state
function pollGamepads() {
    var gamepads = navigator.getGamepads ? navigator.getGamepads() : (navigator.webkitGetGamepads ? navigator.webkitGetGamepads : []);
    for (var i = 0; i < gamepads.length; i++) {
        var gp = gamepads[i];

        if (gp) {
            console.log(gp);
        }
    }
}