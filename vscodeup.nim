# nim compile -d:release -d:ssl --outdir:$HOME/bin --opt:speed vscodeup.nim
import httpclient
import osproc
import strformat
import logging

const
    vscodeUrl = "https://update.code.visualstudio.com/latest/linux-deb-x64/stable"
    vscodeFilePath = "/tmp/vscode-nim.deb"
    installCmd = fmt"sudo dpkg -i {vscodeFilePath}"

var
    logger = newConsoleLogger(fmtStr="[$time] [$levelname] ")

proc downloadVscode() =
    logger.log(lvlInfo, "downloading vscode...")

    var client = newHttpClient()
    client.downloadFile(vscodeUrl, vscodeFilePath)

    logger.log(lvlInfo, "download finished.")

proc installVscode() =
    logger.log(lvlInfo, "installing vscode...")

    if execCmd(installCmd) != 0:
        raise newException(OSError, "installation command failed")

    logger.log(lvlInfo, "installation finished.")

proc run() =
    downloadVscode()
    installVscode()

when isMainModule:
    try:
        run()
    except:
        let err = getCurrentException()
        logger.log(lvlError, fmt("{err.name}: {err.msg}"))
