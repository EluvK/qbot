# qbot

QQChatBot with GPT api 

用在 QQ 的机器人，咱就不写英文了，累。

Rust 实现，抽象了 cqhttp 的收发消息、openai ai 的消息对象，具有私聊好友管理功能、预设机器人角色、指令等功能。

原理上是接受 [cqhttp](https://github.com/Mrs4s/go-cqhttp) 的 local websocket 消息，处理其中的消息，构造出调用 openai api 的调用，得到返回结果后发送回答。

新增了支持通过socks代理调用openai的api，主要方便直接在国内的服务器上挂QQ bot

欢迎 点 Star⭐ , Fork 二次修改 ，提 Issues，提 PR ~ 

注：不负责任何关于 openai key 和 proxy 的问题。

## 如何使用

1. 下载 [cqhttp](https://github.com/Mrs4s/go-cqhttp/releases)
2. 下载 [qbot](https://github.com/EluvK/qbot/releases)
3. 开启 cqhttp （使用 2 local websocket，更多的还是看cqhttp官方吧）
4. 开启 qbot，生成配置文件
5. 编辑配置文件 config.json ，再次开启 qbot : `nohup ./qbot &`

## 配置文件

``` JSON
{
    "websocket": "ws://localhost:8080/ws",
    "proxy": "",
    "api_key": "sk-xxx",
    "bot_qq": 123,
    "root_qq": 456
}
```

## 功能

目前支持:

- [x] 单条消息回复群聊里的 @ 问题, 连续上下文记录 ✔
- [x] 私聊消息，连续上下文记录 ✔
- [x] # 机器人指令，更换prompt内容、清理上下文记录等。
- [x] #sudo 机器人指令，黑名单，强制清理上下文记录等。
- [ ] 计算语句tokon usage，并自动清理

## 参考文档

* https://platform.openai.com/docs/guides/chat
* https://docs.go-cqhttp.org/reference/data_struct.html#post-type

## LICENSE

MIT, 开源，feel free 拿去随便造。但是能留下个Star⭐或者 fork 就更好了~