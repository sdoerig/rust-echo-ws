# rust-echo-ws

A very primitiv and dumb echo service used to tackle down very specific problem on one of our hosts. Consumes and produces JSON.

## Routes 
POST /ibis/rest/rc/setTaskLock/{id}/true

## Example
`curl POST -H "Content-Type: application/json" -d '{"jaco123":"balastime 123"}' http://127.0.0.1:8088/ibis/rest/rc/setTaskLock/1115/true
{"jaco123":"balastime 123","echoedAt":"Thu, 19 Sep 2019 16:11:03 +0000"}`

Note - if `Content-Type` is set to anything else than `application/json` it will not produce any output.
