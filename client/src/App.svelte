<script>
    import Player from "./Player.svelte";
    import Logo from "./Logo.svelte";

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

    function dayLabel(day) {
        const today = new Date().toDateString();
        const yesterday = new Date(Date.now() - 86400000).toDateString();
        if (day === today) return "Today";
        if (day === yesterday) return "Yesterday";
        return day;
    }

    function goupSearch(items) {
        return [{name: "Search results", items: items}];
    }

    async function getItems(query) {
        const url = base + 'news' + (query ? `?search=${query}` : '');

        const res  = await fetch(url);
        const data = await res.json();

        const grouped = (query ? goupSearch(data) : groupByDay(data));

        return grouped;
    }

    let query = $state(undefined);

    const search = (e) => {
        e.preventDefault();
        query = e.target.query.value;
    };

    let data = $derived(getItems(query));

</script>

<header class="container">
    <hgroup>
        <h1 class="brand"><Logo/> News</h1>
        <p>Aggregated stories, summarized and read aloud</p>
    </hgroup>
    <form role="search" onsubmit={search}>
        <input type="search" name="query" placeholder="Search news..."/>
        <button type="submit">Search</button>
    </form>
</header>

<main class="container">
    {#await data}
        <p aria-busy="true">Loading...</p>
    {:then data }

        {#each data as day }
        <section>
            <h2>{dayLabel(day.name)}</h2>

            {#each day.items as item}
                <article>
                    <header>
                        <Player text={item.content}/>
                        <details>
                            <summary>{item.title}</summary>
                            <p class="content">{item.content}</p>
                        </details>
                    </header>
                    <footer>
                        <small>
                            {#each item.sources as source}
                                <a href="{source}" target="_blank">{new URL(source).host.split('.').reverse()[1]}</a>
                            {/each}
                            <a href="{item.link}" target="_blank">{item.source}</a>
                            &middot; {new Date(item.pub_date).toUTCString()}
                        </small>
                    </footer>
                </article>
            {:else}
                <p class="msg">No news for {day.name}</p>
            {/each}
        </section>
        {/each}

    {/await}

</main>

<style>
    h1.brand {
        display: flex;
        align-items: center;
        gap: 0.35em;
    }
</style>
