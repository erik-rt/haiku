<script lang="ts">
	import type { PageData } from './$types';
	import type { Haiku } from '$lib/types';

	// Get todos from page load
	export let data: PageData;
	let haikus = data.haikus;

	// Delete todo
	async function deleteHaiku(id: number) {
		await fetch(`http://127.0.0.1:8000/delete/${id}`, { method: 'POST' });
		haikus = haikus.filter((haiku) => haiku.id !== id);
	}

	// Update todo
	async function updateHaiku(haiku: Haiku) {
		await fetch(`http://127.0.0.1:8000/update/${haiku.id}`, {
			method: 'PATCH',
			headers: {
				'Content-type': 'application/json'
			},
			body: JSON.stringify(haiku)
		});
	}
</script>

<div class="m-auto mt-16">
	<h1
		class="h1 text-center bg-gradient-to-br from-red-500 to-yellow-500 bg-clip-text text-transparent box-decoration-clone"
	>
		Haikus
	</h1>

	<div class="max-w-screen-md mx-auto">
		<form action="http://127.0.0.1:8000/create" method="POST">
			<input
				class="input p-4 my-8"
				name="content"
				type="text"
				placeholder="Write a haiku."
				autocomplete="off"
			/>
		</form>

		<div class="space-y-4">
			{#each haikus as haiku}
				<div class="flex items-center justify-between p-4 bg-surface-800 rounded-lg gap-4">
					<input class="input" type="text" bind:value={haiku.content} />

					<div class="flex gap-2">
						<button class="btn variant-filled-secondary" on:click={updateHaiku(haiku)}
							>Update</button
						>
						<button class="btn variant-filled-primary" on:click={deleteHaiku(haiku.id)}
							>Delete</button
						>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
