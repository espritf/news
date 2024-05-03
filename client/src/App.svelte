<script>
    import Player from "./Player.svelte";

    let base = import.meta.env.VITE_API_URL;

    function groupByDay(items) {
        const days = {};
        items.forEach(item => {
            const date = new Date(item.pub_date);
            const day = date.toDateString();
            if (!days[day]) {
                days[day] = [];
            }
            days[day].push(item);
        });
        return Object.entries(days).map(([name, items]) => ({name, items}));
    }

    async function getItems(query) {
        const url = base + 'news' + (query ? `?search=${query}` : '');

        const res  = await fetch(url);
        const data = await res.json();

        const grouped = groupByDay(data);

        return grouped;
    }

    let query;

    const search = (e) => query = e.target.query.value;

    $: data = getItems(query);

</script>

<main>
    <h1>news</h1>

    <!--<div id="search">-->
        <!--<form on:submit|preventDefault={search}>-->
            <!--<input type="text" name="query"/>-->
            <!--<button type="submit">Search</button>-->
        <!--</form>-->
    <!--</div>-->

    {#await data}
        <p>Loading...</p>
    {:then data }

        {#each data as day }
        <div>
            <h2>{day.name}</h2>
            <div>
                {#each day.items as item}
                    <div>
                        <p>
                            <Player text="{item.title}"/>
                            <span>
                                {item.title}
                            </span>
                            <br/>
                            <small>
                                {#each item.sources as source}
                                    <a href="{source}" target="_blank">{new URL(source).host.split('.').reverse()[1]}</a>
                                {/each}
                                <a href="{item.link}" target="_blank">{item.source}</a>
                                {new Date(item.pub_date).toUTCString()}
                            </small>
                        </p>
                    </div>
                {:else}
                    <div class="msg">No news for {day.name}</div>
                {/each}
            </div>
        </div>
        {/each}

    {/await}

</main>
