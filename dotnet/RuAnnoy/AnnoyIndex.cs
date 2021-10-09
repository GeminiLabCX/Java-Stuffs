using System;
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
            Ty