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
                alerts</strong>, and change the default values for updating all collected
                data!
            </div>

            <h1 class="subtitle">
                Understanding the data!
            </h1>

            <div class="block">
                If you're not into computer science or anything like that, those
                units may feel a bit hard to read or understand, and that's ok! In order
                to give you the possibility to grasp all the data you're visualizing,
                it would be cool to give some context to things.
            </div>

            <div class="block">
                Inside all the parts of the software, there are units that relate to the amount
                of data being sent/received on your machine. That's why we have the Download/
                Upload fields on the Hosts Table! The bigger the number, the higher the amount
                of data being consumed by your network. In this software, data is labeled in
                <strong>MegaBytes (mB)</strong>.
            </div>

            <div class="block">
                I hope this software can help people that have trouble finding out the bottlenecks
                of their systems, so they can be easily eliminated.
                Enjoy the tool!
            </div>
        </div>
    </div>
</div>
