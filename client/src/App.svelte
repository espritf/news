<script>
    let base = import.meta.env.VITE_API_URL;

    async function getUsers(day = 0) {
        const url = base + 'news' + (day ? '/' + day : '');
        console.log(url);

        const res  = await fetch(url);
        const data = await res.json();
        return data;
    }

    const days = [
        {ago: 0, name: "today"},
        {ago: 1, name: "yesterday"},
    ];
</script>

<main>
    <h1>news</h1>

    {#each days as day}
    <div>
        <h2>{day.name}</h2>
        <div>
            {#await getUsers(day.ago)}
                <p>Loading...</p>
            {:then news}
                {#each news as item}
                    <div>
                        <p>
                            {item.title}
                            <br/>
                            <small>
                                <a href="{item.link}" target="_blank">{item.source}</a>
                                {new Date(item.pub_date).toUTCString()}
                            </small>
                        </p>
                    </div>
                {:else}
                    <div class="msg">No news for {day.name}</div>
                {/each}
            {/await}
        </div>
    </div>
    {/each}

</main>
