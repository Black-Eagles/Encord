<script lang="ts">
	export let name: string;
	let random_number: number = 0;
	let min: number = 0;
	let max: number = 10;

	function rand() {
		// get the plain text result of /api/random/0/10 as a number
		fetch("/api/random/" + min + "/" + max)
			.then(response => response.text())
			.then(text => {
				random_number = parseInt(text);
			});

	}
	rand();

	let ws = new WebSocket("ws://localhost:8080/ws/");
	ws.onmessage = function (event) {
		console.log(event.data);
	};
	ws.onopen = function (event) {
		ws.send("Hello World!");
	};

</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	<input type=number bind:value={min}> to <input type=number bind:value={max}> <button on:click="{rand}">new number</button>
	<h2>{random_number}</h2>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>