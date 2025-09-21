// JavaMemory.java
public class JavaMemory {
    static class Leaky {
        private byte[] big = new byte[10_000_000]; // ~10MB
    }

    public static void main(String[] args) throws InterruptedException {
        for (int i = 0; i < 5; i++) {
            Leaky l = new Leaky();
            // l goes out of scope at the end of loop iteration, so eligible for GC
        }

        System.out.println("Requesting GC...");
        System.gc(); // hint to GC; not guaranteed
        Thread.sleep(1000); // give GC a moment in simple runs
        System.out.println("Exiting");
    }
}
