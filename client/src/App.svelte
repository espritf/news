<script>
    let url = import.meta.env.VITE_API_URL;

    async function getUsers() {
        const res  = await fetch(url);
        const data = await res.json();
        return data;
    }
</script>

<main>
    <h1>News</h1>
    {#await getUsers()}
        <p>Loading...</p>
    {:then news}
        {#each news as item}
            <div>
                <h4>{item.title}</h4>
                <small>{new Date(item.pub_date).toUTCString()}</small>
            </div>
        {/each}
    {/await}
</main>
