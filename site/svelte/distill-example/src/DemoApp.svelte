<script>
	import Scatterplot from './Scatterplot.svelte';
	import data from './data.js';
    function handleClick() {
        for (let i = 0; i < data.a.length; i++) {
            let temp = data.a[i].x;
            data.a[i].x = data.a[i].y;
            data.a[i].y = temp;
        }
    }
</script>

<style>
	.chart {
		width: 100%;
		max-width: 640px;
		height: calc(100% - 4em);
		min-height: 280px;
		max-height: 480px;
		margin: 0 auto;
	}
</style>

<div class="chart">
	<Scatterplot points={data.a}/>
	<Scatterplot points={data.b}/>
	<Scatterplot points={data.c}/>
	<Scatterplot points={data.d}/>

    <button on:click={handleClick}>
        Change Data
    </button>
</div>
