using System;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    internal static class NativeMethods
    {
        const string DLLPATH = @"annoy_rs_ffi";

        [DllImport(DLLPATH, EntryPoint = "load_annoy_index", CharSet = CharSet.Ansi)]
        internal static extern IntPtr LoadAnnoyIndex(
            string path,
            Int32 dimension,
            IndexType indexType);

        [DllImport(DLLPATH, EntryPoint = "free_annoy_index", CharSet = CharSet.Ansi)]
        internal static extern void FreeAnnoyIndex(IntPtr