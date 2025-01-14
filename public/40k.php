<?php include('inc/header.php'); ?>

<div class="panel panel-primary">
    <div class="panel-heading">
        <h3 class="panel-title">Input</h3>
    </div>
    <div class="panel-body">
        <p>Go into BattleScribe, and click "Save the Roster":</p>
        <p><img src="/butan.png" alt="BattleScribe Save button" width="300px"/></p>
        <p>Then put the ROSZ/ROS file here and wait for a little bit.</p>
        <form method="post" enctype="multipart/form-data" action="/post.php">
        <div class="form-group">
            <div class="col-sm-offset-2 col-sm-10">
                <input type="file" name="list">
            </div>
        </div>
<!--
        <p>&nbsp;</p>
        <p>You can also use the GW app, I guess. Use this button instead.</p>
        <div class="form-group">
            <div class="col-sm-offset-2 col-sm-10">
                <input type="file" name="gw_list">
            </div>
        </div>

        <p>&nbsp;</p>
        <p>In either case, here are some options you can use:</p>
-->
        <div class="form-group">
            <div class="col-sm-offset-2 col-sm-10">
                <div class="checkbox">
                    <input type="checkbox" name="big_data_sheet_appreciator">BIG SHEETS (one per page, portrait - if unchecked, two per page, landscape)
                </div>
                <div class="checkbox">
                    <input type="checkbox" name="dedupe">Skip duplicates? (will only print one sheet for each <em>identical</em> unit - 10 Guard with a Plasma gun is not considered a dulicate of 10 Guard with a Melta)
                </div>
                <div class="checkbox">
                    <input type="checkbox" name="tracking">Show Crusade tracking
                </div>
                <div class="checkbox">
                    <input type="checkbox" checked="checked" name="reference">Show rules reference
                </div>
            </div>
        </div>
        <div class="form-group">
            <div class="col-sm-offset-2 col-sm-10">
                <input type="submit" value="pres" class="btn btn-default">
            </div>
        </div>
        </form>
    </div>
</div>

<div class="panel panel-primary">
    <div class="panel-heading">
        <h3 class="panel-title">Output</h3>
    </div>
    <div class="panel-body">
        <p>You should get a prompt to download a PDF that looks something like this:</p>
        <p><img src="/output_roster.png" alt="Output data roster example" style="width:100%"/></p>
        <p>Followed by a bunch of these:</p>
        <p><img src="/output.png" alt="Output data card example" style="width:100%"/></p>
        <p>Or, in Crusade mode:</p>
        <p><img src="/output_crusade.png" alt="Output crusade card example" style="width:100%"/></p>
    </div>
</div>

<?php include('inc/footer.php'); ?>
