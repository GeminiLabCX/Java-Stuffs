﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    public class AnnoyIndex : DisposeBase, IAnnoyIndex
    {
        private IntPtr _indexPtr;

        private AnnoyIndex(
            IntPtr indexPtr,
            int dimension,
            IndexType type)
        {
            _indexPtr = indexPtr;
            Dimension = dimension;
            Type = type;

            Size = NativeMethods.GetSize(_indexPtr);
        }

        public int Dimension { get; }

        public ulong Size { get; }

        public IndexType Type { get; }

        public static IAnnoyIndex? Load(
            string path,
            int dimension,
            IndexType type)
        {
            var indexPtr = NativeMethods.LoadAnnoyIndex(path, dimension, type);
            if (indexPtr != IntPtr.Zero)
            {
                return new AnnoyIndex(indexPtr, dimension, type);
            }
            else
            {
                return null;
            }
        }

        public IReadOnlyList<float> GetItemVector(ulong itemIndex)
        {
            if (_indexPtr == IntPtr.Zero)
            {
                throw new ObjectDisposedException("index");
            }

            var itemVector = new float[Dimension];
            NativeMethods.GetItemVector(_indexPtr, itemIndex, itemVector);
            return itemVector;
        }

        public AnnoyIndexSearchR