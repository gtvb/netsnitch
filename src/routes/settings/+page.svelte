<script>
    import { goto } from "$app/navigation";
    import Navbar from "../../components/Navbar.svelte";
    import { settingsStore } from "../../utils/settingsStore.js";

    let fetchDataDelay = $settingsStore.delay;
    let usageLimitUnits = [
        { id: 1, text: "gB"},
        { id: 2, text: "mB"},
        { id: 3, text: "kB"}
    ];
    let usageLimit;
    let selectedUsageLimitUnit;

    function updateSettings() {
        if(!usageLimit || !selectedUsageLimitUnit.text) {
            alert("Error");
            return;
        }

        settingsStore.set({
            delay: fetchDataDelay,
            usage: (usageLimit ? usageLimit : 0),
            unit: selectedUsageLimitUnit.text
        });

        goto("/");
    }
</script>

<div class="container">
    <Navbar />
    <div class="column is-half is-offset-one-quarter">
        <div class="section box mt-6">
            <div class="field">
                <label class="label">Update Rate (Seconds)</label>
                <div class="control">
                    <input bind:value={fetchDataDelay} class="input" type="number" placeholder="Update Rate">
                </div>
            </div>

            <label class="label">Network usage limit (optional)</label>
            <div class="field has-addons">
              <p class="control is-expanded">
                <input bind:value={usageLimit} class="input" type="number" placeholder="Ex: 40">
              </p>
              <p class="control">
                <span class="select">
                  <select bind:value={selectedUsageLimitUnit}>
                      {#each usageLimitUnits as unit}
                        <option value={unit}>
                            {unit.text}
                        </option>
                    {/each}
                  </select>
                </span>
              </p>
            </div>

            <div class="field is-grouped">
                <div class="control">
                    <button on:click|preventDefault={updateSettings} class="button is-link">Submit</button>
                </div>
            </div>
        </div>
    </div>
</div>
