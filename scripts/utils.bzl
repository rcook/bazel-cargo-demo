"""
Utility functions
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def load_http_archive(archive):
    """
    Load HTTP archive

    Args:
        archive: archive info
    """
    name = archive["name"]
    tag = archive["tag"]

    sha256 = archive["sha256"]

    strip_prefix = archive.get("strip_prefix")
    if strip_prefix != None:
        strip_prefix = strip_prefix.format(name = name, tag = tag)

    urls = [u.format(name = name, tag = tag) for u in archive["urls"]]

    http_archive(
        name = name,
        sha256 = sha256,
        strip_prefix = strip_prefix,
        urls = urls,
    )
