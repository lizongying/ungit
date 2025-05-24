# ungit

ungit是一個下載git倉庫的工具，可以僅下載特定分支、標籤，不會下載整個歷史，所以速度會很快。

靈感來源於`degit`。部分項目建議使用`degit`下載項目，但大陸地區可能會遇到網絡問題，本工具可以有效解決這個問題。

[EN](./README.md)

## 下載

[release](https://github.com/lizongying/ungit/releases)

## 使用

- 使用方法基本同`degit`

```shell
ln -s ~/Download/ungit_aarch64-apple-darwin /usr/local/bin/degit
```

```shell
degit lizongying/ungit
```

- 做了簡化，僅支持以下形式：

```shell
degit lizongying/ungit

degit lizongying/ungit#main       # branch
degit lizongying/ungit#v0.1.6    # release tag
```

## 讚賞

![image](./screenshots/appreciate.png)