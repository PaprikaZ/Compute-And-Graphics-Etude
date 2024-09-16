<#
.SYNOPSIS
This compile shader script call the shader compiler to compile shader scripts.

.DESCRIPTION
This powershell script will call the glsl shader compiler to compile predefined
shader script in current entry project. The glsl shader compiler is provided
from vulkan sdk. The vulkan sdk come from https://www.lunarg.com/vulkan-sdk/.

.PARAMETER GlslCompilerBinFilePath
The glsl compiler bin file path

.EXAMPLE
.\compile-shader.ps1
    -GlslCompilerBinFilePath C:\Path\To\VulkanSDK\x.x.x.x\Bin\glslc.exe
#>

param (
    [Parameter(Mandatory=$True, Position=0)]
    [string] $GlslCompileBinFilePath
)

[string[]] $predefinedShaderScriptAndByteCodeFileNameS = @(
    "triangle-red.vert", "triangle-red.vert.spv",
    "triangle-red.frag", "triangle-red.frag.spv",
    "triangle-color.vert", "triangle-color.vert.spv",
    "triangle-color.frag", "triangle-color.frag.spv",
    "triangle-dynamic.vert", "triangle-dynamic.vert.spv"
)

$thisScriptFilePath = $MyInvocation.MyCommand.Path
$thisScriptDirectory = (Get-Item $thisScriptFilePath).Directory
$thisEntryDirectory = $thisScriptDirectory.Parent
$entryShaderDirectory = Join-Path -Path $thisEntryDirectory -ChildPath "source-shader"

function Main {
    Write-Host "Working shader directory $entryShaderDirectory"

    for ($index = 0; $index -lt ($predefinedShaderScriptAndByteCodeFileNameS.Count / 2); $index++) {
        $shaderScriptFileNameIndex = $index * 2
        $shaderByteCodeFileNameIndex = $shaderScriptFileNameIndex + 1
        $shaderScriptFileName = $predefinedShaderScriptAndByteCodeFileNameS[$shaderScriptFileNameIndex]
        $shaderByteCodeFileName = $predefinedShaderScriptAndByteCodeFileNameS[$shaderByteCodeFileNameIndex]
        $shaderScriptFilePath = Join-Path -Path $entryShaderDirectory -ChildPath $shaderScriptFileName
        $shaderByteCodeFilePath = Join-Path -Path $entryShaderDirectory -ChildPath $shaderByteCodeFilename
        $compileCommand = ". '$GlslCompileBinFilePath' $shaderScriptFilePath -o $shaderByteCodeFilePath"
        Write-Host "Compiling shader $shaderScriptFileName to $shaderByteCodeFileName"
        #Write-Host $compileCommand
        Invoke-Expression -Command $compileCommand
    }
}

Main

#$glslCompileBinFilePath = "C:\VulkanSDK\1.3.224.1\Bin\glslc.exe"