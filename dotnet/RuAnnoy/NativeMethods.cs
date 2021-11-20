using System;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    internal static class NativeMethods
    {
        const string DLLPATH = @"annoy_rs_ffi";

        [DllImport(DLLPATH, EntryPoint = "load_an