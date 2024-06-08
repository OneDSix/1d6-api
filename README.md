# 1d6-api

The public REST API for 1D6.\
Found here is Accounts, Mods, Servers, etc.\
Written entirely in [Rust](https://rust-lang.org), and hosted on [Shuttle](https://www.shuttle.rs/).

A lot of this project is based off [Labrinth](https://github.com/modrinth/labrinth), [Modrinth's](https://modrinth.com/) backend services.\
Full credit to them for a large majority of the code and its layout.\
As such, this is licensed under [AGPL v3.0](LICENSE), the same license as Labrinth's.

## For Developers

The full API spec is located in the [wiki](../../wiki/).\
A webhook service to track events like uptime, uploads, and blog posts is in development.\
I'll leave the job of making API wrappers, Discord bots, etc. to you.

## For Contributors

If adding another migration, make sure the file name is `YYYY_MM_DD_Description.sql`, as that makes it a little more organized.
