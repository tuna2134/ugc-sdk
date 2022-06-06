from setuptools import setup
from setuptools_rust import RustExtension

setup_requires = [
    'setuptools-rust>=0.11.1,<0.12',
    'wheel',
]

setup(
    name="ugc-sdk",
    version="0.1.0",
    packages=["ugc_sdk"],
    rust_extensions=[RustExtension("ugc_sdk.ugc_sdk_core")],
    setup_requires=setup_requires,
    include_package_data=True,
)