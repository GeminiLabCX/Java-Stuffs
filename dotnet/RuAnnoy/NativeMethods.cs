﻿using System;
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
        internal static extern void FreeAnnoyIndex(IntPtr index);

        [DllImport(DLLPATH, EntryPoint = "get_dimension", CharSet = CharSet.Ansi)]
        internal static extern int GetDimension(IntPtr index);

        [DllImport(DLLPATH, EntryPoint = "get_size", CharSet = CharSet.Ansi)]
        internal static extern ulong GetSize(IntPtr index);

        [DllImport(DLLPATH, EntryPoint = "get_item_vector", CharSet = CharSet.Ansi)]
        internal static extern void GetItemVector(IntPtr index, ulong itemIndex, [Out] float[] item