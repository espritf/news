<script>
    import { untrack } from 'svelte';
    let { text } = $props();
    let state = $state('play');

    const synthesis = window.speechSynthesis;
    const utterance = new SpeechSynthesisUtterance(untrack(() => text));

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

<button class="round" aria-label="Play/pause" onclick={toggleState}>
    <i class="fa-solid fa-{state}"></i>
</button>