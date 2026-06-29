# Preparing a new version

1. Update dependencies:

    ```
    $ cargo update
    ```

2. Make your changes to the code and merge them to the `main` branch.

3. Update the version number in `Cargo.toml`.

4. Commit the version update:

    ```
    $ git commit -am "Update the version to X.Y.Z"
    ```

5. Tag the latest commit with the new version number:

    ```
    $ git tag -a vX.Y.Z -m "Version X.Y.Z"
    ```

    Make sure to prefix the version in the tag name with "v" for "version".

6. Push the version tag to the remote repository:

    ```
    $ git push --follow-tags
    ```

    If you're using several remote repositories, such as origin and upstream, make sure to push the tag to all of them.


# Packaging and distributing on Crates.io

1. If you are publishing to Crates.io for the first time on this system, log into your account:

    ```
    $ cargo login
    ```

    You can manage your login tokens in your account settings: <https://crates.io/me>.

2. Publish the latest version to Crates.io:

    ```
    $ cargo publish
    ```
