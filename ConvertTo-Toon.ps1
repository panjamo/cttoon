function ConvertTo-Toon {
    [CmdletBinding()]
    param(
        [Parameter(ValueFromPipeline = $true)]
        $InputObject,

        [int]$Depth = 2,

        [ValidateSet('comma', 'tab', 'pipe')]
        [string]$Delimiter,

        [int]$Spaces,

        [ValidateSet('off', 'on', 'safe')]
        [string]$KeyFolding,

        [int]$FlattenDepth,

        [string]$OutFile
    )

    begin {
        $collection = @()
    }

    process {
        $collection += $InputObject
    }

    end {
        if (-not (Get-Command cttoon -ErrorAction SilentlyContinue)) {
            throw "cttoon was not found. Install it via: cargo install cttoon"
        }

        $json = $collection | ConvertTo-Json -Depth $Depth

        $cttoonArgs = @()
        if ($Delimiter)    { $cttoonArgs += '-d', $Delimiter }
        if ($PSBoundParameters.ContainsKey('Spaces'))       { $cttoonArgs += '-s', $Spaces }
        if ($KeyFolding)   { $cttoonArgs += '-k', $(if ($KeyFolding -eq 'on') { 'safe' } else { $KeyFolding }) }
        if ($PSBoundParameters.ContainsKey('FlattenDepth')) { $cttoonArgs += '-f', $FlattenDepth }

        $prevEncoding = [Console]::OutputEncoding
        [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
        $toon = $json | cttoon @cttoonArgs
        [Console]::OutputEncoding = $prevEncoding

        if ($OutFile) {
            $toon | Set-Content -Encoding UTF8 $OutFile
        }
        else {
            $toon
        }
    }
}
