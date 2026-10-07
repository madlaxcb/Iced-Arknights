# 病毒扫描报告（VIRUS-SCAN）

扫描对象：`hud-ui 0.1.0` 发行包中的全部可执行文件。

## 扫描环境

| 项目 | 值 |
|---|---|
| 扫描引擎 | ClamAV `clamscan` 1.4.3 |
| 病毒特征库 | ClamAV 本机已安装库（共 3,628,131 条签名）；本次 freshclam 因 `/var/log/clamav/freshclam.log` 被占用未能更新 |
| 扫描时间 | 2026-10-07 00:47:29 – 00:47:59（本机时区） |
| 扫描命令 | `clamscan linux-x86_64/gallery linux-x86_64/sample-app windows-x86_64/gallery.exe windows-x86_64/sample-app.exe` |

## 逐文件结果

| 文件 | SHA-256 | 结果 |
|---|---|---|
| `linux-x86_64/gallery` | `6d0175ca3ea236dc7aa9264ca23c821e28f30e6492add049191f4c6a6dd523c2` | OK（未发现威胁） |
| `linux-x86_64/sample-app` | `8ad128c86b428170799cd0eeb92ec87725050010368eef5a8c0117395b1cda35` | OK（未发现威胁） |
| `windows-x86_64/gallery.exe` | `1f63ab6e51f1be1c4da332477bb209e6feafa2a26660c5ee8a07dbc3979253ee` | OK（未发现威胁） |
| `windows-x86_64/sample-app.exe` | `fc62f5607da56ff70c7ecfbcf7c824dbd43c0f7b1f550109e40f0e0f4057a6fd` | OK（未发现威胁） |

汇总：扫描文件 4，**感染文件 0**。完整汇总见同目录 `CHECKSUMS.sha256`
（可用 `sha256sum -c CHECKSUMS.sha256` 校验下载完整性）。

## 说明

- 以上为单一引擎（ClamAV）在本机构建环境中的扫描结果，仅作参考，
  不构成任何安全担保。
- 本程序为未签名的 Rust 交叉编译产物，部分杀毒软件可能对无签名可执行文件
  产生启发式误报；如遇告警，建议先核对 SHA-256 是否与本报告一致，
  再将文件提交至 [VirusTotal](https://www.virustotal.com/) 多引擎复核。
