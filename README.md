# QOTD

An implementation of [RFC 865](http://tools.ietf.org/html/rfc865).

### Limitations

- IPv4 only
- TCP only (UDP not yet implemented)
- Bound to `127.0.0.1` (mitigates "pingpong" attack risk)
- No error handling
- No usage or error messages (e.g. if not run as root, can't bind to port)
- Not configurable
- Same quote every day

### License

Released under the terms of the [MIT License](http://opensource.org/licenses/MIT):

```
Copyright (c) 2015 Mike English <mike.english@gmail.com>
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```
