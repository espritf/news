<script>
    let { size = 32 } = $props();
</script>

<svg width={size} height={size * 63.5 / 81} viewBox="7.5 23 81 63.5" xmlns="http://www.w3.org/2000/svg">
    <circle cx="26" cy="30" r="7" fill="#2E7CD6"/>
    <circle cx="48" cy="30" r="7" fill="#F5C518"/>
    <circle cx="70" cy="30" r="7" fill="#D64545"/>
    <path d="M 10 52 C 24 44, 40 46, 48 54 L 48 84 C 40 76, 24 74, 10 82 Z" fill="none" stroke="currentColor" stroke-width="5" stroke-linejoin="round"/>
    <path d="M 86 52 C 72 44, 56 46, 48 54 L 48 84 C 56 76, 72 74, 86 82 Z" fill="none" stroke="currentColor" stroke-width="5" stroke-linejoin="round"/>
</svg>
