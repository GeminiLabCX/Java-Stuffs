/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package com.github.hanabi1224.RuAnnoy

import java.util.Arrays
import kotlin.test.*

class AnnoyIndexTest {
        @Test
        fun testAnnoyIndex() {
                if (!System.getenv("JITPACK").isNullOrBlank()) {
                        return
                }

                val indexPath = "$pwd/src/test/resources/index.angular.5d.ann"
                val index = AnnoyIndex.tryLoad(indexPath, 5, IndexType.Angular)
                assertTrue(index != null, "indexPath: $indexPath")
                assertEquals(100, index.size)
                index.use {
                        assertEquals(5, index.dimension)
                        val v3 = index.getItemVector(3)
                        assertTrue(
                                        v3.toTypedArray()
                                                        .contentDeepEquals(
                                                                        floatArrayOf(
                                                                                                        -0.38846132159233093f,
                                                                                                        0.8791206479072571f,
                                                                                                        0.05800916627049446f,
                                                                                                        0.8664266467094421f,
                                                                                                        0.40251824259757996f,
                                                                                        )
                                                                                        .toTypedArray()
                                                        ),
                                        Arrays.toString(v3)
                        )
                        val v0 = index.getItemVector(0)
                        var nearest = index.getNearest(v0, 5, -1, true)
                        assertTrue(
                                        nearest.idList
                                                        .toTypedArray()
                                                        .contentDeepEquals(
                                                                        longArrayOf(
                                                                                                        0,
                                                                                                        4,
                                                                                                        37,
                                                                                                        61,
                                                                                                        29
                                                                                        )
                                                                                        .toTypedArray()
                                                        ),
                                        Arrays.toString(nearest.idList)
                        )
                        assertEquals(true, nearest.distanceIncluded)
                        assertTrue(
                                        nearest.distanceList
                                                        .toTypedArray()
                                                        .contentDeepEquals(
                                                                        floatArrayOf(
                                                                                                        0.0f,
                                                                                                        0.41608825f,
                                                                                                        0.5517523f,
                                                                                                        0.7342095f,
                                                                                                        0.7592962f
                                                                                        )
                                                                                        .toTypedArray()
                                                        ),
                                        Arrays.toString(nearest.distanceList)
                        )
                        nearest = index.getNearestToItem(0, 5, -1, false)
                        assertTrue(
                                        nearest.idList
                                                        .toTypedArray()
                                                        .contentDeepEquals(
                                                                        longArrayOf(
                                                                                                        0,
                                                                                                        4,
                                                                                                        37,
                                                                                                        61,
                                                                                                        29
                                                                                        )
                                                                                        .toTypedArray()
                                                        ),
                                        Arrays.toString(nearest.idList)
                        )
                        assertEquals(false, nearest.distanceIncluded)
                        assertEquals(0, nearest.distanceList.size)
                }
        }

        companion object {
                val pwd = System.getProperty("user.dir")
        }
}
