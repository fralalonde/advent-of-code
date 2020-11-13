import java.nio.file.FileSystems;
import java.nio.file.Files;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import lombok.val;

public class A {
    public static void main(String[] args) throws Exception {
        val words = Files.lines(FileSystems.getDefault().getPath("data"))
                .map(l -> l.split(" ")).collect(Collectors.toList());
                		
        val rawcount = words.stream().mapToLong(w -> w.length).toArray();
        val distinct = words.stream().mapToLong(w -> Stream.of(w).distinct().count()).toArray();
        
        int c = 0;
        int d = 0;
        for (int z = 0; z < rawcount.length; z++) {
        	if (rawcount[z] == distinct[z]) {
        		System.out.println(rawcount[z]);
        		c++;
        	} else {
        		d++;
        	}
        }
        System.out.println(c + "\t" + d + "\t" + rawcount.length);
    }
}