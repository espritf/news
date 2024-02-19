<script>
    export let text;
    let state = 'play';

    const synthesis = window.speechSynthesis;
    const utterance = new SpeechSynthesisUtterance(text);

    utterance.onstart = function() {
        state = 'pause';
    };
    utterance.onend = function() {
        state = 'play';
    };

    function toggleState() {
        if (state === 'play') {
            synthesis.speak(utterance);
        } else {
            state = 'play';
            synthesis.cancel();
        }
    }

</script>

<button class="round" on:click={toggleState}>
    <i class="fa-solid fa-{state}"/>
</button>