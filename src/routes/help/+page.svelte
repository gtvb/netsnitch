<script>
    import Navbar from "../../components/Navbar.svelte";
</script>

<div class="container">
    <Navbar />
    <div class="column is-half is-offset-one-quarter">
        <div class="section box">
            <h1 class="title is-4">
                Welcome to NetSnitch!
            </h1>

            <div class="block">
                The purpose of this program is to help you safely monitor
                your network activities during every moment of your day!
            </div>
            <div class="block">
                The software provides an easy-to-use interface, where you can
                currently visualize the following data:
            </div>

            <div class="block">
                &bull; Total network usage<br/>
                &bull; Application usage rankings<br/>
                &bull; Protocol usage rankings<br/>
                &bull; IP addresses currently connected to your machine<br/>
            </div>

            <div class="block">
                You can also change some of the settings to include a <strong>usage limit
                alert</strong>, and change the default values for updating all collected
                data! Enjoy!
            </div>
        </div>
    </div>
</div>
