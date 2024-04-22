
Robotframework remote library interface for Rust
================================================

This is an approach to build a Robotframework remote library interface for Rust.

Status: Proof of concept

See also for the Python reference implemenation:

https://github.com/robotframework/RemoteInterface

Official Docu:

https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#remote-library-interface


Observed API calls
==================

Traced with wireshark on the Python Example.

- get_library_information // for the newer single call API
- get_keyword_names // for the dynamic library API
- get_keyword_tags // for the dynamic library API
- get_keyword_documentation // for the dynamic library API
- get_keyword_arguments  // for the dynamic library API
- get_keyword_types // for the dynamic library API
- run_keyword

TODO: We should swith to the newer api. Is there a useful example?

See also:

https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#toc-entry-640
