using System;
using System.Collections.Generic;

namespace RuAnnoy
{
    public interface IAnnoyIndex : IDisposable
    {
        int Dimension { get; }

        ulong Size { get; }

        IReadOnlyList<float> GetItemVector(ulong itemIndex);

        AnnoyIndexSearchResult GetNearest(
            IReadOnlyList<float> queryVector,
            uint nResult,
            int searchK,
         