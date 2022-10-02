<script>
	import { afterUpdate, onDestroy, onMount } from "svelte";
	import { invoke } from "@tauri-apps/api";
	import { appWindow } from "@tauri-apps/api/window";

	let scale = "4";
	let style = "Anime";
	let disableSelect = false;
	let disableOpen = false;
	let disableConvert = true;
	let outputs = [];
	let logger;
	let filePaths = [];
	let fileType = "none";
	let unlisten;
	let isLoading = false;

	onMount(async () => {
		unlisten = await appWindow.listen("output-log", event => {
			outputs = [...outputs, event.payload];
		});
	});

	onDestroy(() => {
		unlisten();
	});

	afterUpdate(() => {
		if (logger) {
			logger.scroll({ top: logger.scrollHeight, behavior: "smooth" });
		}
	});

	let modelOptions = [
		{
			label: "Anime",
			disabled: false,
		},
		{
			label: "Standard",
			disabled: false,
		},
	];

	let scaleOptions = [
		{
			label: "2",
			disabled: false,
		},
		{
			label: "3",
			disabled: false,
		},
		{
			label: "4",
			disabled: false,
		},
	];

	async function selectFiles() {
		disableButtons(); // Temporarily disable
		await invoke("select_files")
			.then(res => {
				// Reset after file selection
				resetButtons();

				// res[0] are the file paths; res[i] is the file type
				if (res[0].length == 0) {
					disableConvert = true;
					outputs = [...outputs, "No files selected.."];
					return;
				}
				filePaths = [...res[0]];
				outputs.push("=== SELECTED FILES ===");
				res[0].forEach(path => {
					outputs.push(path.split("\\").pop());
				});
				outputs = [...outputs, "======================"];

				// Set file type
				fileType = res[1];

				disableConvert = false;
				if (fileType == "image") {
					noScaling(true); // Images can't be scaled at the moment
					onlyAnime(false); // Can change model
				}

				if (fileType == "video") {
					onlyAnime(true); // No standard model for videos at the moment
					noScaling(false); // Can change scaling
				}
			})
			// Print errors to output log
			.catch(err => {
				outputs = [...outputs, err];
				// Reset buttons if error
				resetButtons();
			});
	}

	async function openOutputFolder() {
		disableOpen = true;
		await invoke("open_output_folder");
		disableOpen = false;
	}

	async function convert() {
		disableButtons(); // disable temporarily
		console.log(filePaths);
		console.log(scale);
		await invoke("convert", {
			filePaths: filePaths,
			fileType: fileType,
			style: style,
			scale: scale,
		});
		resetButtons();
	}

	function onlyAnime(isEnabled) {
		style = "Anime";
		modelOptions.forEach(model => {
			if (model.label != "Anime") {
				model.disabled = isEnabled;
			}
		});
		modelOptions = modelOptions;
	}

	function noScaling(isEnabled) {
		scale = "4";
		scaleOptions.forEach(scale => {
			if (scale.label != "4") {
				scale.disabled = isEnabled;
			}
		});
		scaleOptions = scaleOptions;
	}

	function disableButtons() {
		isLoading = true;
		disableSelect = true;
		disableOpen = true;
		disableConvert = true;

		modelOptions.forEach(m => {
			m.disabled = true;
		});
		modelOptions = modelOptions;

		scaleOptions.forEach(s => {
			s.disabled = true;
		});
		scaleOptions = scaleOptions;
	}

	function resetButtons() {
		isLoading = false;
		disableSelect = false;
		disableOpen = false;
		disableConvert = true;

		modelOptions.forEach(m => {
			m.disabled = false;
		});
		modelOptions = modelOptions;

		scaleOptions.forEach(s => {
			s.disabled = false;
		});
		scaleOptions = scaleOptions;
	}
</script>

<div class="main-container">
	<div class="output-box">
		<div class="logs-container" bind:this={logger}>
			{#each outputs as output}
				<p>>> {output}</p>
			{/each}
		</div>
	</div>
	<div class="buttons-container">
		<button on:click={selectFiles} disabled={disableSelect}>
			Select Files
		</button>
		<button on:click={openOutputFolder} disabled={disableOpen}>
			Open Output Folder
		</button>
		<button on:click={convert} disabled={disableConvert}>Convert</button>
		<div class="model-selector">
			<div>Style:</div>
			{#each modelOptions as m}
				<label>
					<input
						type="radio"
						id={m.label}
						name={m.label}
						value={m.label}
						bind:group={style}
						disabled={m.disabled}
					/>
					{m.label}
				</label>
			{/each}
		</div>
		<div class="scale-selector">
			<div>Scale:</div>
			{#each scaleOptions as s}
				<label>
					<input
						type="radio"
						id={s.label}
						name={s.label}
						value={s.label}
						bind:group={scale}
						disabled={s.disabled}
					/>
					{s.label}x
				</label>{/each}
		</div>
		{#if isLoading}
			<div class="loader">
				<div />
				<h4>Loading...</h4>
			</div>
		{/if}
	</div>
</div>

<style>
	.loader {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 225px;
	}
	.loader div {
		border: 8px solid #f3f3f3; /* Light grey */
		border-top: 8px solid #555; /* Blue */
		border-radius: 50%;
		width: 120px;
		height: 120px;
		animation: spin 1.5s linear infinite;
		margin-bottom: 8px;
	}
	.loader h4 {
		padding: 0;
		margin: 8px 0 0 0;
	}
	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
	.buttons-container {
		display: flex;
		flex-direction: column;
		width: 250px;
		height: 500px;
	}
	.buttons-container button {
		width: 100%;
		height: 50px;
		margin-bottom: 8px;
	}
	.main-container {
		display: flex;
		justify-content: space-between;
	}
	.output-box {
		border: solid 1px;
		border-radius: 4px;
		width: 500px;
		height: 500px;
	}
	.logs-container {
		font-family: monospace;
		max-height: 100%;
		overflow-y: auto;
	}
	.logs-container p {
		margin: 4px 8px;
	}
	.scale-selector {
		border: solid 1px;
		border-radius: 4px;
		padding: 12px 16px;
		display: flex;
		justify-content: space-between;
	}
	.model-selector {
		border: solid 1px;
		border-radius: 4px;
		padding: 12px 16px;
		display: flex;
		justify-content: space-between;
		margin-bottom: 8px;
	}
</style>
