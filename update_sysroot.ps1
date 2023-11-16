# Get the sysroot using rustc
$SYSROOT = (rustc --print sysroot).Replace("\", "/")

$json = @'
{
    "sysroot_src": "",
    "crates": [
        {
            "root_module": "main.rs",
            "edition": "2021",
            "deps": []
        }
    ]
}
'@

$jsonContent = ConvertFrom-Json -InputObject $json

# Update sysroot_src
$jsonContent.sysroot_src = $SYSROOT + "/lib/rustlib/src/rust/library"

# Convert to JSON and save to rust-project.json
$jsonContent | ConvertTo-Json -Depth 10 | Set-Content rust-project.json
