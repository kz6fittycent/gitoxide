use bstr::ByteSlice;
use git_url::Scheme;

use crate::parse::{assert_url_and, assert_url_roundtrip, url, url_alternate};

#[test]
fn file_path_with_protocol() -> crate::Result {
    assert_url_roundtrip(
        "file:///path/to/git",
        url(Scheme::File, None, None, None, b"/path/to/git"),
    )
}

#[test]
fn file_path_without_protocol() -> crate::Result {
    let url = assert_url_and(
        "/path/to/git",
        url_alternate(Scheme::File, None, None, None, b"/path/to/git"),
    )?
    .to_bstring();
    assert_eq!(url, "/path/to/git");
    Ok(())
}

#[test]
fn no_username_expansion_for_file_paths_without_protocol() -> crate::Result {
    let url = assert_url_and(
        "~/path/to/git",
        url_alternate(Scheme::File, None, None, None, b"~/path/to/git"),
    )?
    .to_bstring();
    assert_eq!(url, "~/path/to/git");
    Ok(())
}
#[test]
fn no_username_expansion_for_file_paths_with_protocol() -> crate::Result {
    assert_url_roundtrip(
        "file://~username/path/to/git",
        url(Scheme::File, None, None, None, b"~username/path/to/git"),
    )
}

#[test]
fn non_utf8_file_path_without_protocol() -> crate::Result {
    let parsed = git_url::parse(b"/path/to\xff/git".as_bstr())?;
    assert_eq!(
        parsed,
        url_alternate(Scheme::File, None, None, None, b"/path/to\xff/git",)
    );
    let url_lossless = parsed.to_bstring();
    assert_eq!(
        url_lossless.to_string(),
        "/path/to�/git",
        "non-unicode is made unicode safe after conversion"
    );
    assert_eq!(url_lossless, &b"/path/to\xff/git"[..], "otherwise it's lossless");
    Ok(())
}

#[test]
fn relative_file_path_without_protocol() -> crate::Result {
    let parsed = assert_url_and(
        "../../path/to/git",
        url_alternate(Scheme::File, None, None, None, b"../../path/to/git"),
    )?
    .to_bstring();
    assert_eq!(parsed, "../../path/to/git");
    let url = assert_url_and(
        "path/to/git",
        url_alternate(Scheme::File, None, None, None, b"path/to/git"),
    )?
    .to_bstring();
    assert_eq!(url, "path/to/git");
    Ok(())
}

#[test]
fn interior_relative_file_path_without_protocol() -> crate::Result {
    let url = assert_url_and(
        "/abs/path/../../path/to/git",
        url_alternate(Scheme::File, None, None, None, b"/abs/path/../../path/to/git"),
    )?
    .to_bstring();
    assert_eq!(url, "/abs/path/../../path/to/git");
    Ok(())
}

#[test]
#[cfg(unix)]
fn url_from_absolute_path() -> crate::Result {
    assert_url_and(
        url::Url::from_directory_path("/users/foo")
            .expect("valid")
            .to_file_path()
            .expect("valid path")
            .to_string_lossy()
            .as_ref(),
        url_alternate(Scheme::File, None, None, None, b"/users/foo/"),
    )?;
    Ok(())
}

mod windows {
    use git_url::Scheme;

    #[test]
    #[cfg(windows)]
    fn url_from_absolute_path() -> crate::Result {
        assert_url_and(
            url::Url::from_directory_path("c:\\users\\1")
                .expect("valid")
                .to_file_path()
                .expect("valid path")
                .to_string_lossy()
                .as_ref(),
            url_alternate(Scheme::File, None, None, None, b"C:\\users\\1\\"),
        )?;
        // A special hack to support URLs on windows that are prefixed with `/` even though absolute.
        assert_url_and(
            "file:///c:/users/2",
            url(Scheme::File, None, None, None, b"c:\\users\\2"),
        )?;
        assert_url_and(
            "file:///c:/users/3/",
            url(Scheme::File, None, None, None, b"c:\\users\\3\\"),
        )?;
        Ok(())
    }

    use crate::parse::{assert_url_and, assert_url_roundtrip, url, url_alternate};

    #[test]
    fn file_path_without_protocol() -> crate::Result {
        let url = assert_url_and(
            "x:/path/to/git",
            url_alternate(Scheme::File, None, None, None, b"x:/path/to/git"),
        )?
        .to_bstring();
        assert_eq!(url, "x:/path/to/git");
        Ok(())
    }

    #[test]
    fn file_path_with_backslashes_without_protocol() -> crate::Result {
        let url = assert_url_and(
            "x:\\path\\to\\git",
            url_alternate(Scheme::File, None, None, None, b"x:\\path\\to\\git"),
        )?
        .to_bstring();
        assert_eq!(url, "x:\\path\\to\\git");
        Ok(())
    }

    #[test]
    fn file_path_with_protocol() -> crate::Result {
        assert_url_roundtrip(
            "file://x:/path/to/git",
            url(Scheme::File, None, None, None, b"x:/path/to/git"),
        )
    }
}
