from setuptools import setup, Extension
from setuptools.command.build_ext import build_ext
import sys
import setuptools
import pybind11

# Add C++11 support
class BuildExt(build_ext):
    def build_extensions(self):
        if sys.platform == 'darwin':  # macOS specific flags
            for ext in self.extensions:
                ext.extra_compile_args = ['-std=c++11']
        super().build_extensions()

ext_modules = [
    Extension(
        "rights_cpp",
        ["bindings.cpp", "rights.cpp"],
        include_dirs=[pybind11.get_include()],
        language='c++',
        extra_compile_args=['-std=c++11'] if sys.platform == 'darwin' else []
    ),
]

setup(
    name="rights_cpp",
    ext_modules=ext_modules,
    install_requires=['pybind11>=2.4.3'],
    setup_requires=['pybind11>=2.4.3'],
    cmdclass={'build_ext': BuildExt},
    zip_safe=False,
) 