# kren-select

Windows 한국어 입력기에서 한글/영어 입력을 전환해주는 툴

- Time & Language > Language & region > Options > Microsoft IME 에서 Compatibility 활성화 후 사용해야합니다.

## Install

```ps1
scoop bucket add lewohy-bucket https://github.com/lewohy/lewohy-bucket
scoop install kren-select
```

## Usage

```ps1
Usage: kren-select.exe [MODE]

Arguments:
  [MODE]  [possible values: kr, en]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 현재 입력기 모드 확인

```ps1
kren-select
```

### 한글 입력 모드로 변경

```ps1
kren-select kr
```

### 영어 입력 모드로 변경

```ps1
kren-select en
```

### With im-select.nvim

```lua
{
    'keaising/im-select.nvim',
    opts = {
        default_im_select = "en",
        default_command   = "kren-select.exe",
    },
}
```

### With Obsidian

#### Vim IM Control

![Obsidian](https://raw.githubusercontent.com/lewohy/kren-select/master/images/obsidian.png)

