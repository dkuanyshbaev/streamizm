function audio_play() {
    var ap = document.getElementById('videoPlayer');
    var btn = document.getElementById('mute_unmute');

    if (ap.muted) {
        ap.muted = false;
        btn.firstChild.data = "Mute";
    } else {
        ap.muted = true;
        btn.firstChild.data = "Unmute";
    }

    return false;
}

function switch_view(button_id){
    document.getElementById(button_id).disabled = true;
    document.getElementById(disabled_button_id).disabled = false;
    disabled_button_id = button_id;
    player.attachSource("dash/" + button_id + "/stream.mpd");
}

function full_screen(){
    if (
        document.fullscreenElement ||
        document.webkitFullscreenElement ||
        document.mozFullScreenElement ||
        document.msFullscreenElement
    ) {
        if (document.exitFullscreen) {
            document.exitFullscreen();
        } else if (document.mozCancelFullScreen) {
            document.mozCancelFullScreen();
        } else if (document.webkitExitFullscreen) {
            document.webkitExitFullscreen();
        } else if (document.msExitFullscreen) {
            document.msExitFullscreen();
        }
    } else {
        element=$('.video_container').get(0);
        if (element.requestFullscreen) {
            element.requestFullscreen();
        } else if (element.mozRequestFullScreen) {
            element.mozRequestFullScreen();
        } else if (element.webkitRequestFullscreen) {
            element.webkitRequestFullscreen(Element.ALLOW_KEYBOARD_INPUT);
        } else if (element.msRequestFullscreen) {
            element.msRequestFullscreen();
        }
    }
}
