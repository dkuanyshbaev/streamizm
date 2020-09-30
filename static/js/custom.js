function audio_play() {
    var ap = document.getElementById('audioPlayer');
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
