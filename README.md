# ungit

ungit is a tool for downloading Git repositories. It allows you to download only a specific branch or tag without
downloading the entire history, resulting in faster downloads.

The inspiration for this tool comes from `degit`. Some projects recommend using `degit` to download projects, but users
in mainland China may encounter network issues. This tool effectively addresses that problem.

[漢](./README_HANT.md)

## Download

[release](https://github.com/lizongying/ungit/releases)

## Usage

- The usage is basically the same as `degit`.
- It has been simplified and only supports the following formats:

```shell
ungit user/repo

ungit user/repo#dev       # branch
ungit user/repo#v1.2.3    # release tag
```

## Support for the project

![image](./screenshots/appreciate.png)