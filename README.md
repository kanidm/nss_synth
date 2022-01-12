
# NSS Synth

NSS Synth is a module that synthesises uids/gids into real groups. This means that when you have
a container with bare uids/gids, these are able to resolve to a concrete user name and group name
so that calls like getpwnam() function correctly.

## Install

```
cargo build --release
sudo cp target/release/libnss_synth.so /usr/lib64/libnss_synth.so.2
```

Edit /etc/nsswitch as:

```
passwd:		compat synth
group:		compat synth
shadow:		compat
```

Remember, the synth module MUST be last!

## How do I know it's working?

Before:

```
# id
uid=1400 gid=1400 groups=1400
OR
id: ‘1400’: no such user
# getent passwd 1400
# echo $?
2
```

After:

```
# id
uid=1400(1400) gid=1400(1400) groups=1400(1400)
# getent passwd 1400
1400:x:1400:1400:1400:/var/lib/empty:/usr/bin/false
```

