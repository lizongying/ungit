# ungit

ungit is a tool for downloading Git repositories. It allows you to download only a specific branch or tag without
downloading the entire history.

The inspiration for this tool comes from `degit`. Some projects recommend using `degit` to download projects, but users
in mainland China may encounter network issues. This tool effectively addresses that problem.

[漢](./README_HANT.md)

## Download

[release](https://github.com/lizongying/ungit/releases)

## Usage

- The usage is basically the same as `degit`.

```shell
ln -s ~/Download/ungit_aarch64-apple-darwin /usr/local/bin/degit
```

```shell
degit lizongying/ungit
```

- It has been simplified and only supports the following formats:

```shell
degit lizongying/ungit

degit lizongying/ungit#main       # branch
degit lizongying/ungit#v0.1.6    # release tag
```

## Support for the project

![image](./screenshots/appreciate.png)