function audio_play() {
    var ap = document.getElementById('audio_player');
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
