import java.nio.file.FileSystems;
import java.nio.file.Files;
import java.util.Arrays;
import java.util.HashSet;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import lombok.val;

public class B {
    public static void main(String[] args) throws Exception {
        val words = Files.lines(FileSystems.getDefault().getPath("data"))
                .map(l -> Stream.of(l.split(" "))
                		.map(w -> {
                			byte[] ga = w.getBytes(); 
                			Arrays.sort(ga);
                			return new String(ga);
                		})
                		.collect(Collectors.toList()))
                .collect(Collectors.toList());
                		
        val rawcount = words.stream().mapToLong(w -> w.size()).toArray();
        val distinct = words.stream().map(w -> new HashSet<String>(w)).collect(Collectors.toList());
        
        int c = 0;
        int d = 0;
        for (int z = 0; z < rawcount.length; z++) {
    		System.out.println(words.get(z));
        	if (rawcount[z] == distinct.get(z).size()) {
        		System.out.println("oui\t" + rawcount[z]);
        		c++;
        	} else {
        		System.out.println("non\t" + rawcount[z] + "\t" +  distinct.get(z));
        		d++;
        	}
        }
        System.out.println(c + "\t" + d + "\t" + rawcount.length);
    }
}